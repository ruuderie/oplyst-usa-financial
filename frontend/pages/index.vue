<script setup>
import { ref } from "vue";

const industries = ref([
  { id: 1, name: 'Real estate', description: 'Partnering with property professionals for value-creating solutions.' },
  { id: 2, name: 'Healthcare', description: 'Energizing the pulse of healthcare with financial vitality.' },
  { id: 3, name: 'Manufacturing', description: 'Powering production lines with efficient financial engines.' },
  { id: 4, name: 'Retail', description: 'Revitalizing retail with creative, customer-focused finance.' },
  { id: 5, name: 'Financial services', description: 'Fostering growth in finance with innovative lending strategies.' },
  { id: 6, name: 'Technology', description: 'Fueling the future of tech with targeted financial tools.' },
]);

const services = ref([
  { id: 1, name: 'Real estate loans', description: 'Tailored financing solutions for your property ambitions.' },
  { id: 2, name: 'Equipment loans', description: 'Invest in your business growth with strategic equipment financing.' },
  { id: 3, name: 'Lines of credit', description: 'Flexible access to funds whenever your business needs it.' },
  { id: 4, name: 'Accounts Receivable finance', description: 'Boost your cash flow by leveraging your receivables.' },
  { id: 5, name: 'Startups', description: 'Kickstart your dream venture with our bespoke financing options.' },
  { id: 6, name: 'Merchant Cash Advance', description: 'Meet your short-term needs with our quick and easy cash advances.' },
]);
</script>

<template>
<div>
  <Head>
    <Title>Oplyst International | Innovation, Integrity, Impact</Title>
    <Link 
      rel="preconnect" 
      href="https://fonts.googleapis.com"/>
    <Link 
      rel="preconnect" 
      href="https://fonts.gstatic.com" 
      crossorigin/>
    <Link 
      href="https://fonts.googleapis.com/css2?family=Inter:wght@100&display=swap" 
      rel="stylesheet"> </Link>        
  </Head>
  <NuxtLayout>
    <section class="container is-small">
      <!--<nuxt-img class="img" 
        src=""
        alt="Oplyst International - Pioneering Commercial Lending Solutions" 
      /> -->
    </section > 
    <section id="hero" class="hero is-large">
      <div class="hero-body">
        <h1 class="title">
          Oplyst International, LLC
        </h1>
        <h3 class="subtitle">
          Commercial Lending Solutions for Forward-Thinking Enterprises
        </h3>
        <p>
          <ul>
            <li>
              <NuxtLink class="tag" to="https://linkedin.com/in/oplyst-international">
                @oplyst-international
              </NuxtLink>
            </li>
          </ul>
        </p>
      </div>
    </section>
    <section class="container is-small py-3">
      <div class="level">                    
        <div class="level-item has-text-centered">
          <div class="box">
            <input class="input is-small py-4 mx-3 my-3" type="name" placeholder="First Name">
            <input class="input is-small py-4 mx-3" type="email" placeholder="email@yourcompany.com">
            <button class="button is-medium mx-3 my-3 is-responsive">
              Stay Informed
            </button>
          </div>
        </div>
      </div>
    </section>
    <section class="container is-medium py-6">
      <center>
        <p class="title py-2">
          Redefining Industry Boundaries with Integrated Commercial Lending Solutions.
        </p>
        <p class="subtitle py-2">
          We deliver innovative technology solutions, efficient logistics services, and strategic financial products to empower enterprises.
        </p>
      </center>     
    </section>
    <section class="container is-medium py-6 has-text-centered">
      <h2>Our Services</h2>
      <div class="row">
        <div class="col-lg-4" v-for="service in services" :key="service.id">
          <div class="card">
            <div class="card-body">
              <h5 class="card-title">{{ service.name }}</h5>
              <p class="card-text">{{ service.description }}</p>
            </div>
          </div>
        </div>
      </div>
    </section>
    <section class="container is-medium py-6 has-text-centered">
      <h2>Industries We Serve</h2>
      <div class="row">
        <div class="col-lg-4" v-for="industry in industries" :key="industry.id">
          <div class="card">
            <div class="card-body">
              <h5 class="card-title">{{ industry.name }}</h5>
              <p class="card-text">{{ industry.description }}</p>
            </div>
          </div>
        </div>
      </div>
    </section>    
    <span class="mb-2 pb-2"/>
    <section class="section is-medium is-mobile has-text-centered">
      <h1 class="title">Our Values</h1>
      <ul>
        <li class="subtitle">Innovation</li>
        <li class="subtitle">Simplicity</li>
        <li class="subtitle">Excellence</li>
        <li class="subtitle">Privacy</li>
        <li class="subtitle">Client Service</li>
        <li class="subtitle">Integrity</li>
        <li class="subtitle">Teamwork</li>
      </ul>
    </section>
    <section class="section is-medium is-mobile">
      <div class="content is-large">
        <center> <h3>Oplyst International </h3> </center>
        <p>At Oplyst International, we are innovators, disruptors, and pioneers. Our unique combination of technology, logistics, and financial advisory allows us to provide holistic solutions to complex challenges. We do business with integrity, fostering long-term partnerships based on mutual trust and respect.</p>
        <p>With a firm commitment to simplicity, quality, and service, we prioritize client satisfaction and champion a future where businesses thrive. Oplyst International is more than just a company - we are shaping the future. Schedule a chat with us today to learn more about our vision, mission, and the services we offer.</p>  
      </div>
      <div class="has-text-centered py-3">
        <button class="button is-large is-responsive">
            Schedule a Discovery Call
        </button>
      </div>
    </section>
  </NuxtLayout>
</div>
</template>

<style lang="scss">
</style>
