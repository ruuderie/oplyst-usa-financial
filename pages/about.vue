<script setup>
import { ref, onMounted } from "vue";

const post = {
  summary: "This is a summary",
  title: "About Ruud",
  featured_image: "https://example.com/image.png",

}

</script>

<template>
<div>
      <Head>
        <Title> {{ post.title }} </Title>
        <Meta name="description" :content="post.summary" />
        <Meta property="og:title" :content="post.title" />
        <Meta name="twitter:title" :content="post.title" />
        <Meta
          property="og:description"
          :content="post.summary"
        />
        <Meta
          name="twitter:description"
          :content="post.summary"
        />
        <Meta
          property="og:image"
          :content="post.featured_image"
        />
        <Meta
          property="twitter:image"
          :content="post.featured_image"
        />
      </Head>
      <NuxtLayout>
        <div class="container is-fluid">
          <section id="hero" class="hero is-medium">
                  <div class="hero-body">
                      <h1 class="title">
                          {{post.title}}
                      </h1>
                  </div>
        </section>
        <div class="content is-large">
                <h3 class="has-text-centered">Ruud Erie </h3> 
                <p>Ruud Erie is a highly respected software architect and engineer who specializes in developing high performance applications that meet rapidly changing consumer demands and business needs. With his deep expertise in software architecture and agile-friendly digital transformation, he is a sought-after consultant who regularly delivers results across the globe.</p>
                <p>Ruud has extensive experience working with top-tier companies on a wide range of projects, including enterprise applications and cloud-based solutions. He is particularly skilled in using enterprise platforms such as Salesforce, AWS, Google Cloud, and Microsoft Azure to deliver optimal results.</p>
                <p>With a proven track record of enterprise implementations, Ruud began his career freelancing while working as poker dealer at a casino. One day he built an application to help his managers give him more overtime and ended up working as a software engineer for a leading consulting company in Hartford, Connecticut.</p>
                <p>He then began doing Product Development in the Salesforce space. He was part of three very successful start-ups including Evariant which eventually got aquired in 2020. After working in enterprise product development. Ruud expanded into consulting full-time and helping other great companies produce great products and successful digital transformations using his expertise. </p>
                <p>Ruud has held a range of key roles throughout his career, including software engineer, software architect, solution architect, technical manager, project manager, and consultant.</p>
                <p>Whether you need hands-on support or strategic leadership in your software projects, Ruud is an ideal partner. He is available to work with your team in a variety of capacities, from keynotes and consulting to training and more. </p>
                <p>To learn more about how Ruud can help your organization succeed, schedule a chat with him today.</p>  
            </div>
          </div>
          <div class="has-text-centered py-3 mx-3 my-3">
                <button class="button is-large is-responsive">
                        Schedule a free discovery call
                </button>
            </div>
      </NuxtLayout>

    </div>

  </template>

<style>

</style>