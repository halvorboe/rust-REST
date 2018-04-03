<template>
  <div id='#app' class='container'>
    <br><br>
    <div class='row'>
      <div class='col-lg-12'>
        <input v-model="filter"  class="form-control" type="text" placeholder="Filter by category">
      </div>
    </div>
    <br><br>
    <div class='row'>
      <div v-for='article in articles' :key='article._id' class='col-lg-6' v-if='filter.length == 0 || article.category.startsWith(filter)'>
        <h2>{{ article.title }}</h2>
        <p>{{ article.body }}</p>
        <p><b>Category: </b>{{ article.category }}</p>
        <p>
          <button class='btn' disabled>{{ article.votes }}</button>
          <button class='btn' v-on:click='vote(article, "true")'>+</button>
          <button class='btn'  v-on:click='vote(article, "false")'>-</button>
        </p>
        <h3>Comments</h3>
        <section class="form">
          <div class="field">
            <label class="label">Title</label>
            <div class="control">
              <input v-model="article.comment_title"  class="form-control" type="text" placeholder="Title">
            </div>
          </div>
          <div class="field">
            <label class="label">Body</label>
            <div class="control">
              <input v-model="article.comment_body"  class="form-control" type="text" placeholder="Body">
            </div>
          </div>
          <br>
          <button class='btn' v-on:click='comment(article._id, article.comment_title, article.comment_body)'>Publiser</button>
          <hr>
        </section>
        <ul v-for='comment in article.comments' :key='comment._id'>
          <p><b>{{ comment.title }}</b><br>{{ comment.body }}</p>
        </ul>

      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'
let config = {
  headers: {
    'Content-Type': 'text/plain'
  },
  responseType: 'text'
}

export default {
  name: 'Index',
  created () {
    this.refresh()
  },
  watch: {
    // call again the method if the route changes
    '$route': 'refresh'
  },
  methods: {
    refresh () {
      this.fetch().then((response) => {
        let data = []
        for (let i = 0; i < response.length; i++) {
          data.push(response[i])
        }
        data = data.sort((x, y) => x.votes < y.votes)
        this.articles = data
      })
    },
    fetch () {
      return axios.get('http://127.0.0.1:4000/api/article/')
        .then((response) => {
          return response.data
        })
        .catch((error) => {
          console.log(error)
        })
    },
    comment (id, title, body) {
      let _ = this
      axios.post('http://127.0.0.1:4000/api/article/comment/', {
        title: title,
        body: body,
        article_id: id
      }, config)
        .then(function (response) {
          console.log(response)
          _.refresh()
        })
        .catch(function (error) {
          console.log(error)
        })
    },
    vote (article, vote) {
      let _ = this
      axios.post('http://127.0.0.1:4000/api/article/vote/', {
        article_id: article._id,
        vote: vote
      }, config)
        .then(function (response) {
          console.log(response)
          _.refresh()
        })
        .catch(function (error) {
          console.log(error)
        })
    }
  },
  data () {
    return {
      filter: '',
      articles: []
    }
  }
}
</script>
