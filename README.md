# Zed GitHub Context Server
This extension provides a Model Context Server for GitHub, for use with the Zed AI assistant.

## Configuration
To use the extension, you will need to provide the context server with your GitHub personal access token in your Zed settings.json:

```json
{
  "context_servers": {
    "github-context-server": {
      "settings": {
        "github_personal_access_token": "your_personal_access_token_here"
      }
    }
  }
}
```
