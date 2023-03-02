package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

var (
	Version string = "bootstrapper-demo"
)

func health(w http.ResponseWriter, r *http.Request) {
	if os.Getenv("BAD_VALUE") != "" {
		fmt.Printf("Bad value found, aborting.\n")
		w.WriteHeader(500)
		w.Write([]byte("error"))
	} else {
		w.WriteHeader(200)
		w.Write([]byte("ok"))
	}
}

func main() {

	fileServer := http.FileServer(http.Dir("./static")) // New code
	http.Handle("/", fileServer)
	http.Handle("/healthz", http.HandlerFunc(health))

	fmt.Printf("Starting server at port 8080\n")
	fmt.Printf("Version %s\n", Version)
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal(err)
	}
}
