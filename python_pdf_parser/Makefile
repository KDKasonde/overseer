
SHELL := /bin/bash
.PHONY: setup-venv # Creates a virtual environment for the project
setup-venv:
		rm -rf .venv
		rm -f .python-version
		uv venv 
		echo 'layout uv' > .envrc
		direnv allow
# Removing .python-version is to cleanup the approach to venvs before uv was used, so uv works correctly

