package twitter

import (
	"context"
	"datagen/gen"
	"datagen/sink"
	"encoding/json"
	"fmt"
	"math/rand"
	"time"

	"github.com/brianvoe/gofakeit/v6"
)

type tweetData struct {
	CreatedAt string `json:"created_at"`
	Id        string `json:"id"`
	Text      string `json:"text"`
	Lang      string `json:"lang"`
}

type twitterEvent struct {
	Data   tweetData   `json:"data"`
	Author twitterUser `json:"author"`
}

type twitterUser struct {
	CreatedAt string `json:"created_at"`
	Id        string `json:"id"`
	Name      string `json:"name"`
	UserName  string `json:"username"`
	Followers int    `json:"followers"`
}

func (r *twitterEvent) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (data, author) values (%s, %s);",
		"twitter", r.Data.objectString(), r.Author.objectString())
}

func (r *twitterUser) objectString() string {
	return fmt.Sprintf("('%s'::TIMESTAMP, '%s', '%s', '%s')", r.CreatedAt, r.Id, r.Name, r.UserName)
}

func (r *tweetData) objectString() string {
	return fmt.Sprintf("('%s'::TIMESTAMP, '%s', '%s', '%s')", r.CreatedAt, r.Id, r.Text, r.Lang)
}

func (r *twitterEvent) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return "twitter", data
}

type twitterGen struct {
	faker *gofakeit.Faker
	users []*twitterUser
}

func NewTwitterGen() gen.LoadGenerator {
	faker := gofakeit.New(0)
	users := make(map[string]*twitterUser)
	for len(users) < 100000 {
		id := faker.DigitN(10)
		if _, ok := users[id]; !ok {
			endYear := time.Now().Year() - 1
			startYear := endYear - rand.Intn(8)

			endTime, _ := time.Parse("2006-01-01", fmt.Sprintf("%d-01-01", endYear))
			startTime, _ := time.Parse("2006-01-01", fmt.Sprintf("%d-01-01", startYear))
			users[id] = &twitterUser{
				CreatedAt: faker.DateRange(startTime, endTime).Format(gen.RwTimestampLayout),
				Id:        id,
				Name:      fmt.Sprintf("%s %s", faker.Name(), faker.Adverb()),
				UserName:  faker.Username(),
				Followers: gofakeit.IntRange(1, 100000),
			}
		}
	}
	usersList := []*twitterUser{}
	for _, u := range users {
		usersList = append(usersList, u)
	}
	return &twitterGen{
		faker: faker,
		users: usersList,
	}
}

func (t *twitterGen) generate() twitterEvent {
	id := t.faker.DigitN(19)
	author := t.users[rand.Intn(len(t.users))]

	wordsCnt := t.faker.IntRange(10, 20)
	hashTagsCnt := t.faker.IntRange(0, 2)
	hashTags := ""
	for i := 0; i < hashTagsCnt; i++ {
		hashTags += fmt.Sprintf("#%s ", t.faker.BuzzWord())
	}
	sentence := fmt.Sprintf("%s%s", hashTags, t.faker.Sentence(wordsCnt))
	return twitterEvent{
		Data: tweetData{
			Id:        id,
			CreatedAt: time.Now().Format("2006-01-02 15:04:05.07"),
			Text:      sentence,
			Lang:      gofakeit.Language(),
		},
		Author: *author,
	}
}

func (t *twitterGen) KafkaTopics() []string {
	return []string{"twitter"}
}

func (t *twitterGen) Load(ctx context.Context, outCh chan<- sink.SinkRecord) {
	for {
		record := t.generate()
		select {
		case <-ctx.Done():
			return
		case outCh <- &record:
		}
	}
}
