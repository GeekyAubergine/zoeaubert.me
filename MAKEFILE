build_and_deploy:
	git fetch origin
	nvm use
	npm i
	npm run build
	cp -R _site/. /var/www/zoeaubert.me/
	