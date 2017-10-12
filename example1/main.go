package main

import "fmt"
import "os"

func main() {
  fmt.Printf("Hello there are %v args. %v\n", len(os.Args), os.Args);
}
