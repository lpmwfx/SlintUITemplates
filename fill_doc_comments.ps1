$ErrorActionPreference = "Stop"

function Split-Words([string]$name) {
    if ([string]::IsNullOrWhiteSpace($name)) { return @() }
    $text = $name -replace "([a-z0-9])([A-Z])", '$1 $2'
    $text = $text -replace "[-_]", " "
    $text = $text -replace "\s+", " "
    return @($text.Trim().ToLower().Split(" ", [System.StringSplitOptions]::RemoveEmptyEntries))
}

function To-Phrase([string]$name) {
    $words = Split-Words $name
    if ($words.Count -eq 0) { return $name }
    return ($words -join " ")
}

function Capitalize([string]$text) {
    if ([string]::IsNullOrEmpty($text)) { return $text }
    return $text.Substring(0, 1).ToUpper() + $text.Substring(1)
}

function Has-DocAbove([System.Collections.Generic.List[string]]$lines, [int]$index) {
    for ($j = $index - 1; $j -ge 0; $j--) {
        $trim = $lines[$j].Trim()
        if ($trim -eq "") { continue }
        return $trim.StartsWith("///") -or $trim.StartsWith("//!")
    }
    return $false
}

function Slint-Doc([string]$kind, [string]$name, [string]$line) {
    switch ($kind) {
        "component" { return "/// ``$name`` component." }
        "struct" { return "/// ``$name`` struct." }
        "global" { return "/// ``$name`` global values." }
        "property" {
            if ($line -match "^\s*in-out property") { return "/// Two-way property ``$name``." }
            if ($line -match "^\s*out property") { return "/// Output property ``$name``." }
            if ($line -match "^\s*private property") { return "/// Private property ``$name`` used internally." }
            return "/// Input property ``$name``."
        }
        "callback" { return "/// Callback for ``$name``." }
        default { return "/// ``$name``." }
    }
}

function Rust-Doc([string]$kind, [string]$name) {
    switch ($kind) {
        "mod" { return "/// ``$name`` module." }
        "struct" { return "/// ``$name`` struct." }
        "enum" { return "/// ``$name`` enum." }
        "trait" { return "/// ``$name`` trait." }
        "type" { return "/// ``$name`` type alias." }
        "const" { return "/// ``$name`` constant." }
        "fn" {
            if ($name -eq "new") { return "/// Create a new instance." }
            if ($name -eq "build") { return "/// Build and return the configured value." }
            if ($name -eq "run") { return "/// Run the current workflow." }
            if ($name -match "^set_(.+)$") { return "/// Set $(To-Phrase $Matches[1])." }
            if ($name -match "^get_(.+)$") { return "/// Get $(To-Phrase $Matches[1])." }
            if ($name -match "^is_(.+)$") { return "/// Return whether $(To-Phrase $Matches[1])." }
            if ($name -match "^has_(.+)$") { return "/// Return whether this has $(To-Phrase $Matches[1])." }
            if ($name -match "^load_(.+)$") { return "/// Load $(To-Phrase $Matches[1])." }
            if ($name -match "^build_(.+)$") { return "/// Build $(To-Phrase $Matches[1])." }
            if ($name -match "^apply_(.+)$") { return "/// Apply $(To-Phrase $Matches[1])." }
            if ($name -match "^parse_(.+)$") { return "/// Parse $(To-Phrase $Matches[1])." }
            if ($name -match "^from_(.+)$") { return "/// Create a value from $(To-Phrase $Matches[1])." }
            if ($name -match "^to_(.+)$") { return "/// Convert this value to $(To-Phrase $Matches[1])." }
            if ($name -match "^on_(.+)$") { return "/// Register a handler for $(To-Phrase $Matches[1])." }
            return "/// ``$name``."
        }
        default { return "/// ``$name``." }
    }
}

function Is-GeneratedDoc([string]$trim) {
    return (
        $trim -match '^/// (Input|Output|Two-way|Private) property ' -or
        $trim -match '^/// Callback for ' -or
        $trim -match '^/// Callback fired for ' -or
        $trim -match '^/// ``.+`` (component|struct|global values|module|enum|trait|type alias|constant)\.$' -or
        $trim -match '^/// (?:[A-Za-z]{1,3}\s+){2,}(component|struct|global values|module|enum|trait|type alias|constant)\.$' -or
        $trim -match '^/// ``.+``\.$'
    )
}

function Update-File([string]$path, [scriptblock]$matcher, [bool]$attributeAware) {
    $content = [System.IO.File]::ReadAllText($path)
    $newline = if ($content.Contains("`r`n")) { "`r`n" } else { "`n" }
    $lines = [System.Collections.Generic.List[string]]::new()
    foreach ($line in ($content -split "`r?`n")) { [void]$lines.Add($line) }

    $changed = $false
    for ($i = 0; $i -lt $lines.Count; $i++) {
        $match = & $matcher $lines[$i]
        if (-not $match) { continue }
        $target = $i
        if ($attributeAware) {
            while ($target -gt 0 -and $lines[$target - 1].Trim().StartsWith("#[")) {
                $target--
            }
        }

        $hasDoc = Has-DocAbove $lines $target
        $directAbove = if ($target -gt 0) { $lines[$target - 1].Trim() } else { "" }

        if ($hasDoc) {
            if (-not $attributeAware) { continue }
            if ($i -gt 0 -and (Is-GeneratedDoc $lines[$i - 1].Trim()) -and $target -lt $i) {
                $lines.RemoveAt($i - 1)
                $changed = $true
                $i--
            }
            continue
        }

        if ($directAbove -like '///*' -and (Is-GeneratedDoc $directAbove)) {
            $indent = ([regex]::Match($lines[$target - 1], "^\s*")).Value
            $lines[$target - 1] = $indent + $match.doc
            $changed = $true
            continue
        }

        $indent = ([regex]::Match($lines[$target], "^\s*")).Value
        $lines.Insert($target, $indent + $match.doc)
        $changed = $true
        $i++
    }

    if ($changed) {
        $item = Get-Item -LiteralPath $path
        if ($item.IsReadOnly) {
            $item.IsReadOnly = $false
        }
        [System.IO.File]::WriteAllText($path, ($lines -join $newline), [System.Text.UTF8Encoding]::new($false))
    }
    return $changed
}

$slintMatcher = {
    param([string]$line)
    $trim = $line.Trim()
    if ($trim -match "^export component\s+([A-Za-z0-9_-]+)\b") {
        return @{ doc = Slint-Doc "component" $Matches[1] $trim }
    }
    if ($trim -match "^export struct\s+([A-Za-z0-9_-]+)\b") {
        return @{ doc = Slint-Doc "struct" $Matches[1] $trim }
    }
    if ($trim -match "^export global\s+([A-Za-z0-9_-]+)\b") {
        return @{ doc = Slint-Doc "global" $Matches[1] $trim }
    }
    if ($trim -match "^(?:in|out|in-out|private) property\s*<[^>]+>\s*([A-Za-z0-9_-]+)\b") {
        return @{ doc = Slint-Doc "property" $Matches[1] $trim }
    }
    if ($trim -match "^callback\s+([A-Za-z0-9_-]+)\s*\(") {
        return @{ doc = Slint-Doc "callback" $Matches[1] $trim }
    }
    return $null
}

$rustMatcher = {
    param([string]$line)
    $trim = $line.Trim()
    if ($trim -match "^pub mod\s+([A-Za-z0-9_]+)\s*;") {
        return @{ doc = Rust-Doc "mod" $Matches[1] }
    }
    if ($trim -match "^pub struct\s+([A-Za-z0-9_]+)\b") {
        return @{ doc = Rust-Doc "struct" $Matches[1] }
    }
    if ($trim -match "^pub enum\s+([A-Za-z0-9_]+)\b") {
        return @{ doc = Rust-Doc "enum" $Matches[1] }
    }
    if ($trim -match "^pub trait\s+([A-Za-z0-9_]+)\b") {
        return @{ doc = Rust-Doc "trait" $Matches[1] }
    }
    if ($trim -match "^pub type\s+([A-Za-z0-9_]+)\b") {
        return @{ doc = Rust-Doc "type" $Matches[1] }
    }
    if ($trim -match "^pub const\s+([A-Za-z0-9_]+)\b") {
        return @{ doc = Rust-Doc "const" $Matches[1] }
    }
    if ($trim -match "^pub fn\s+([A-Za-z0-9_]+)\s*\(") {
        return @{ doc = Rust-Doc "fn" $Matches[1] }
    }
    if ($trim -match "^pub\s+([A-Za-z0-9_]+)\s*:") {
        return @{ doc = "/// $(Capitalize (To-Phrase $Matches[1]))."}
    }
    return $null
}

$changed = [System.Collections.Generic.List[string]]::new()

$slintFiles = rg --files ui -g "*.slint"
foreach ($file in $slintFiles) {
    if (Update-File $file $slintMatcher $false) { [void]$changed.Add($file) }
}

$rustRoots = @("src", "examples")
foreach ($root in $rustRoots) {
    $rustFiles = rg --files $root -g "*.rs"
    foreach ($file in $rustFiles) {
        if (Update-File $file $rustMatcher $true) { [void]$changed.Add($file) }
    }
}

$changed | Sort-Object
