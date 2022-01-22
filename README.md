# Simple_Minimizer_Algorithmn
A Simple Minimizer Algorithm that matches reads based on shared minimizers, disregarding order and multiplicity.

Overview: 
It takes in two fasta files. The first file is the references and it is used to create a HashMap of the the minimal k-mers in reference sequences. The second file is the query and the program decides which sequence in the reference the query sequences are more likely the same as.

Detailed Steps
1. The program first reads the two genomes from the reference and creates a vector of minimizer kmers, each representing an l-sized window in the sequence, for each genome
   - The kmers selected are the kmers with the minimal hash value in each l-sized window 
2. The program then takes in the query fasta file and converts each query sequence into a vector of minimizer kmers
3. The vector is compared to vectors representing our reference genomes, and the amount of hits are calculated
Remark: Hits at the current stage only counts matches of kmers, disregarding repeats or order of the kmers 
4. The program then returns a string telling us for each query, which reference it is likely from, or if it is from none of them 

Example Fasta files provided
HepatitisC.fasta: The complete genome of genotype 1 and genotype 7 of Hepatitis C, obtained from NCBI
query.fasta: Subsequences of the two genomes in HepatitisC.fasta, with slight modifications to the ends
  id1: from genotype 7
  id2: from genotype 1
  id3: from genotype 7
  id4: a random sequence
error_query.fasta: sequences from query.fasta, but with more errors introduced 
  id5: id1 with substitutions {T,A, 4}, {C, T, 19}, and {C, G, 46}
  id6: id 2 with an insertion of CATCGT at position 10
  id7: id2 with a deletion of 10 nucleotides at position 10

Possible Improvements:
  - Deal with what happens if there are multiple maximums
  - Write the results as a file
  - Use Enums to select for how we select for minimizers
