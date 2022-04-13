package main

import (
	"fmt"
	"log"
	"net/http"
	"net/http/httputil"
)

func logging(w http.ResponseWriter, r *http.Request) {
	req, err := httputil.DumpRequest(r, true)

	if err != nil {
		fmt.Println(err)
	}

	log.Printf("Request: %s", string(req))
	fmt.Fprintf(w, "%s", string(req))

}

func main() {
	fmt.Println("--- What the header??? ---")
	fmt.Printf("Each request will log the headers and also print the received headers to the response body.\n\n")
	log.Println("Listening on 0.0.0.0:8080.")
	http.HandleFunc("/", logging)
	http.ListenAndServe(":8080", nil)
}
