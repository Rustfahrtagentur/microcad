#!/bin/bash
# Copy the completion script to the directory
sudo cp ./microcad-bash-completion /etc/bash_completion.d/ &&
sudo chown root:root /etc/bash_completion.d/microcad-bash-completion &&
sudo chmod 644 /etc/bash_completion.d/microcad-bash-completion
