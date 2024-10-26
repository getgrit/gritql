import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock
from gritql.installer import find_install, CLIError

def test_find_install_existing_grit():
    with patch('shutil.which', return_value='/usr/local/bin/grit'):
        assert find_install() == Path('/usr/local/bin/grit')

def test_find_install_download_grit_macos():
    with patch('shutil.which', return_value=None), \
         patch('sys.platform', 'darwin'), \
         patch('platform.machine', return_value='arm64'), \
         patch('httpx.Client') as mock_client, \
         patch('tarfile.open'), \
         patch('os.chmod'):

        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.iter_bytes.return_value = [b'mock_data']
        mock_client.return_value.__enter__.return_value.get.return_value = mock_response

        result = find_install()
        assert isinstance(result, Path)
        assert result.name == 'grit'

        # Test the URL that is called
        expected_url = "https://github.com/getgrit/gritql/releases/latest/download/grit-aarch64-apple-darwin.tar.gz"
        mock_client.return_value.__enter__.return_value.get.assert_called_once_with(expected_url, follow_redirects=True)

def test_find_install_download_grit_linux():
    with patch('shutil.which', return_value=None), \
         patch('sys.platform', 'linux'), \
         patch('platform.machine', return_value='x86_64'), \
         patch('httpx.Client') as mock_client, \
         patch('tarfile.open'), \
         patch('os.chmod'):

        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.iter_bytes.return_value = [b'mock_data']
        mock_client.return_value.__enter__.return_value.get.return_value = mock_response

        result = find_install()
        assert isinstance(result, Path)
        assert result.name == 'grit'

        # Test the URL that is called
        expected_url = "https://github.com/getgrit/gritql/releases/latest/download/grit-x86_64-unknown-linux-gnu.tar.gz"
        mock_client.return_value.__enter__.return_value.get.assert_called_once_with(expected_url, follow_redirects=True)

def test_find_install_windows():
    with patch('sys.platform', 'win32'):
        with pytest.raises(CLIError, match="Windows is not supported yet"):
            find_install()
