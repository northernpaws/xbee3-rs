package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

const (
	sourceFile = "commandssrc.rs"
	outputDir  = "commands"
	modFile    = "mod.rs"
)

var (
	variantRegex = regexp.MustCompile(`^\s*([A-Za-z0-9]+)\s*(?:\(([^)]+)\))?\s*,?\s*(?://.*)?$`)
)

func isUpper(r rune) bool {
	return r >= 'A' && r <= 'Z'
}

func isLower(r rune) bool {
	return r >= 'a' && r <= 'z'
}

func toLower(r rune) rune {
	if isUpper(r) {
		return r + 32
	}
	return r
}

func toSnakeCase(s string) string {
	var result strings.Builder
	runes := []rune(s)

	for i := 0; i < len(runes); i++ {
		if i > 0 && isUpper(runes[i]) && (isLower(runes[i-1]) || (i+1 < len(runes) && isLower(runes[i+1]))) {
			result.WriteByte('_')
		}
		result.WriteRune(toLower(runes[i]))
	}

	return result.String()
}

func main() {
	file, err := os.Open(sourceFile)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	if err := os.MkdirAll(outputDir, 0755); err != nil {
		panic(err)
	}

	modOut, err := os.Create(filepath.Join(outputDir, modFile))
	if err != nil {
		panic(err)
	}
	defer modOut.Close()

	scanner := bufio.NewScanner(file)
	inEnum := false

	fmt.Fprintf(modOut, "pub trait Command {\n")
	fmt.Fprintf(modOut, "    fn identifier(&self) -> super::Identifier;\n")
	fmt.Fprintf(modOut, "}\n\n")

	fmt.Println("starting to read", sourceFile)
	for scanner.Scan() {
		line := scanner.Text()

		if strings.Contains(line, "pub enum Commands") {
			inEnum = true
			fmt.Println("found start of commands")
			continue
		}

		if !inEnum || strings.Contains(line, "}") {
			fmt.Println("found end of commands")
			continue
		}

		if strings.TrimSpace(line) == "" || strings.HasPrefix(strings.TrimSpace(line), "//") {
			fmt.Println("ignoring command or blank line")
			continue
		}

		matches := variantRegex.FindStringSubmatch(line)
		if matches == nil {
			fmt.Println("no match found on line")
			continue
		}

		variant := matches[1]
		typeHint := ""
		if len(matches) > 2 {
			typeHint = matches[2]
		}

		fmt.Println("found variant", variant, typeHint)

		fileStem := toSnakeCase(variant)
		filename := fileStem + ".rs"
		path := filepath.Join(outputDir, filename)

		structDef := fmt.Sprintf("pub struct %s", variant)
		if typeHint != "" {
			structDef += fmt.Sprintf("(pub %s);", typeHint)
		} else {
			structDef += ";"
		}
		// structDef += ";"

		// Write each file
		out, err := os.Create(path)
		if err != nil {
			panic(err)
		}

		fmt.Fprintf(out, "use crate::at::{Command, Identifier};\n\n")
		fmt.Fprintln(out, structDef)
		fmt.Fprintln(out)
		fmt.Fprintf(out, "impl super::Command for %s {\n", variant)
		fmt.Fprintf(out, "    fn identifier(&self) -> Identifier {\n")
		fmt.Fprintf(out, "        Identifier::%s\n", variant)
		fmt.Fprintf(out, "    }\n")
		fmt.Fprintln(out, "}")
		fmt.Fprintln(out, "")
		fmt.Fprintf(out, "impl From<%s> for Command<0> {\n", variant)
		fmt.Fprintf(out, "    fn from(cmd: %s) -> Command<0> {\n", variant)
		fmt.Fprintln(out, "        Command{")
		fmt.Fprintf(out, "            identifier: Identifier::%s,\n", variant)
		fmt.Fprintf(out, "            payload: None,\n")
		fmt.Fprintf(out, "            carriage_returns: 1,\n")
		fmt.Fprintln(out, "        }")
		fmt.Fprintln(out, "    }")
		fmt.Fprintln(out, "}")
		out.Close()

		// Update mod.rs
		fmt.Fprintf(modOut, "mod %s;\n", fileStem)
		fmt.Fprintf(modOut, "pub use %s::*;\n\n", fileStem)
	}

	fmt.Println("✔ Done generating command structs and mod.rs")
}
