import pytest
from unittest.mock import patch, MagicMock
from gritql.run import run_cli, apply_pattern

def test_run_cli():
    with patch('gritql.run.find_install', return_value='/path/to/grit'), \
         patch('subprocess.run') as mock_run:

        mock_run.return_value.returncode = 0

        assert run_cli(['test', 'args']) == 0
        mock_run.assert_called_once_with(['/path/to/grit', 'test', 'args'])

def test_apply_pattern():
    with patch('gritql.run.run_cli') as mock_run_cli:
        mock_run_cli.return_value = 0

        assert apply_pattern('test_pattern', ['arg1', 'arg2']) == 0
        mock_run_cli.assert_called_once_with(['apply', 'test_pattern', 'arg1', 'arg2'])

def test_apply_pattern_with_grit_dir():
    with patch('gritql.run.run_cli') as mock_run_cli:
        mock_run_cli.return_value = 0

        assert apply_pattern('test_pattern', ['arg1'], grit_dir='/path/to/grit') == 0
        mock_run_cli.assert_called_once_with(['apply', 'test_pattern', 'arg1', '--grit-dir', '/path/to/grit'])
