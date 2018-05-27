install:
		mkdir -p $(HOME)/bin
		ln -s $(realpath target/release/git-jira) $(HOME)/bin/git-jira
