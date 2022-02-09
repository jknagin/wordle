words=$(cat solutions.txt)
for WORD in $words
do
	./target/release/wordle --guess aesir --secret $WORD --queries queries.txt --solutions solutions.txt
done

