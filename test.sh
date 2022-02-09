words=$(cat sgb-words-sorted.txt)
for WORD in $words
do
	./target/release/wordle --guess aloes --secret $WORD -c --path sgb-words-sorted.txt
done

