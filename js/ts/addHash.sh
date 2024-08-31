hashtag (){
        cd ~/aiWritter/js/ts
        bun getHash.ts $1
}

echo 'export hashtagP=hashtag' >> ~/.bashrc s
