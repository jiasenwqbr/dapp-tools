package main

import "fmt"

func main() {
	fmt.Println("build blockchain in go!")
	bc := NewBlockchain()
	bc.AddBlock("Send 1 BTC to Jason")
	bc.AddBlock("Send 2 BTC to Jason")
	bc.AddBlock("Send 3 BTC to Jason")

	for _, block := range bc.blocks {
		fmt.Printf("Prev. hash: %x\n", block.PrevBlockHash)
		fmt.Printf("Data: %s\n", block.Data)
		fmt.Printf("Hash: %x\n", block.Hash)
		fmt.Println()
	}
}
