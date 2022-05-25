package main

import (
	"encoding/csv"
	"io"
	"log"
	"os"
)

func main () {
	f, err := os.Open("brooklyn.csv")
	data := []string{}

	if err != nil {
		log.Fatal(err)
	}
	
	reader:= csv.NewReader(f)
	reader.LazyQuotes = true

	for {
		col, err := reader.Read()

		if err == io.EOF {
			break	

		}
		
	}
}