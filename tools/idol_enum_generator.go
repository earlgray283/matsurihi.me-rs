package main

import (
	"fmt"
	"log"
	"net/http"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
	"github.com/ettle/strcase"
	"github.com/gojp/kana"
)

func main() {
	pascalIdolNames := []string{}
	idolNames := []string{}

	fmt.Println("pub enum Idol {")
	for i := 1; i <= 52; i++ {
		resp, err := http.Get(fmt.Sprintf("https://imas.gamedbs.jp/mlth/chara/show/%d", i))
		if err != nil {
			log.Fatal(err)
		}
		defer resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			log.Fatal("Response status code was not 200 OK")
		}
		doc, err := goquery.NewDocumentFromReader(resp.Body)
		if err != nil {
			log.Println(err)
		}

		text := doc.Find("#contents-main > section > section > article.d1_3 > div:nth-child(3) > ul > li:nth-child(2)").Text()
		idolFirstSecond := make([]string, 2)
		for i, elm := range strings.Split(text, " ") {
			idolFirstSecond[i] = kana.KanaToRomaji(elm)
		}
		
		pascalIdolName := strcase.ToPascal(strings.Join(idolFirstSecond, "_"))
		pascalIdolNames = append(pascalIdolNames, pascalIdolName)
		fmt.Printf("\t%s = %d,\n", pascalIdolName, i)

		idolNames = append(idolNames, doc.Find("#contents-main > section > section > article.d1_3 > h2").Text())

		time.Sleep(time.Second * 1)
	}
	fmt.Println("}")

	fmt.Println(`
impl std::fmt::Display for Idol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {`)

	for i, pascalIdolName := range pascalIdolNames {
		fmt.Printf(`Idol::%s => write!(f, "%s"),`, pascalIdolName, idolNames[i])
		fmt.Println()
	}

	fmt.Println(`
		}
	}
}`)
}
