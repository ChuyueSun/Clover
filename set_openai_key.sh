#!/bin/bash

# Script to set OpenAI API key in the current environment

# First try to source the private key file if it exists
if [ -f "$HOME/Clover/private_key.sh" ]; then
  source "$HOME/Clover/private_key.sh"
  echo "✅ Loaded API key from private_key.sh"
  exit 0
fi

# If private file doesn't exist, use the placeholder
# IMPORTANT: Replace YOUR_OPENAI_API_KEY with your actual key before using
export OPENAI_API_KEY="YOUR_OPENAI_API_KEY"

# Print confirmation
echo "✅ OPENAI_API_KEY has been set"
echo "Key length: $(echo $OPENAI_API_KEY | wc -c | xargs) characters"
echo ""
echo "⚠️  IMPORTANT: For private usage, create a private_key.sh file with your actual API key"
echo "    This allows you to keep your key out of the git repository"
echo "    Example of private_key.sh content:"
echo "    #!/bin/bash"
echo "    export OPENAI_API_KEY=\"your-actual-api-key-here\""
echo "Use this script with: source set_openai_key.sh"
echo "⚠️  IMPORTANT: Edit this file to add your actual OpenAI API key before using" 