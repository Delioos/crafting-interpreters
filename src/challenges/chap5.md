# side note 
## If you’re so inclined, try using this grammar to generate a few expressions like we did with the breakfast grammar before. Do the resulting expressions look right to you? Can you make it generate anything wrong like 1 + / 3?

-> we cannot generate shi like 1 + / 3 because `+` followed by `/` implies to find a way to `operator operator` but the only way to get an operator is from `binary`, and this operator is wrapped between expressions  
