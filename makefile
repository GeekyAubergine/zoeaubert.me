build_and_deploy:
	git pull
	nvm use
	npm i
	npm run build
	cp -R _site/. /var/www/zoeaubert.me/
	