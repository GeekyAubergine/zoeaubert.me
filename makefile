build_and_deploy:
	git pull
	/root/.nvm/versions/node/v19.4.0/bin/nvm i
	/root/.nvm/versions/node/v19.4.0/bin/nvm run build
	cp -R _site/. /var/www/zoeaubert.me/
	