package tree_sitter_php_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-php"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_php.Language())
	if language == nil {
		t.Errorf("Error loading Php grammar")
	}
}
