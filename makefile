build_and_deploy:
	git pull
	npm i
	npm run build
	cp -R _site/. /var/www/zoeaubert.me/
	