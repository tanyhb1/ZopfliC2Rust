./zopfli test/data/*
mv test/data/*.gz test/results_of_c2rustzopfli/
diff test/results test/results_of_c2rustzopfli/
