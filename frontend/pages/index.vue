<script setup>
import { ref } from "vue";
import Card from "@/components/Card.vue";

const industries = ref([
  { id: 1, name: 'Real estate', description: 'Partnering with property professionals for value-creating solutions.', imageSource: ""},
  { id: 2, name: 'Healthcare', description: 'Energizing the pulse of healthcare with financial vitality.', imageSource: "" },
  { id: 3, name: 'Manufacturing', description: 'Powering production lines with efficient financial engines.', imageSource: "" },
  { id: 4, name: 'Retail', description: 'Revitalizing retail with creative, customer-focused finance.', imageSource: "" },
  { id: 5, name: 'Financial services', description: 'Fostering growth in finance with innovative lending strategies.', imageSource: "" },
  { id: 6, name: 'Technology', description: 'Fueling the future of tech with targeted financial tools.', imageSource: "" },
]);

const services = ref([
  { id: 1, name: 'Real estate loans', description: 'Tailored financing solutions for your property ambitions.', imageSource: "" },
  { id: 2, name: 'Equipment loans', description: 'Invest in your business growth with strategic equipment financing.', imageSource: "" },
  { id: 3, name: 'Lines of credit', description: 'Flexible access to funds whenever your business needs it.', imageSource: "" },
  { id: 4, name: 'Accounts Receivable finance', description: 'Boost your cash flow by leveraging your receivables.', imageSource: "" },
  { id: 5, name: 'Startups', description: 'Kickstart your dream venture with our bespoke financing options.', imageSource: "" },
  { id: 6, name: 'Merchant Cash Advance', description: 'Meet your short-term needs with our quick and easy cash advances.', imageSource: "" },
]);


</script>

<template>
<div class="page-container">
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
      <nuxt-img 
        src="@/assets/david-kiriakidis-bP0q3tTTHS4-unsplash.jpg"
        alt="Oplyst International - Pioneering Commercial Lending Solutions" 
      />
    </section > 
    <section id="hero" class="hero is-large">
      <div class="hero-body">
        <nuxt-img src="@/assets/Oplyst International White logo 800x600.png" alt="Logo"/>

        <h1 class="title">
          Oplyst International, LLC
        </h1>
        <nuxt-img src="~/assets/Oplyst International White logo 800x600.png" alt="Logo" />

        <h3 class="subtitle">
          Commercial Financing Solutions for Forward-Thinking Enterprises
        </h3>
        <p>
          <ul>
            <li>
              <NuxtLink class="tag" to="https://www.linkedin.com/company/oplystusa/">
                @oplyst-international
              </NuxtLink>
            </li>
          </ul>
        </p>
      </div>

      <center>
        <p class="title py-2">
          Commercial Lending Solutions for Real Estate and Business.
        </p>
        <p class="subtitle py-2">
          We deliver innovative technology solutions, efficient logistics services, and strategic financial products to empower enterprises.
        </p>
      </center>     
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
    <section class="container is-medium py-6 has-text-centered">
      <h2 class="has-text-weight-bold">Our Services</h2>
      <div class="row">
        <div class="col-lg-4" v-for="service in services" :key="service.id">
          <card
            :id="service.id.toString()"
            :name="service.name"
            :description="service.description"
            :imageSource="service.imageSource"
          />
        </div>
      </div>
    </section>

    <section class="container is-medium py-6 has-text-centered">
      <h2 class="has-text-weight-bold">Industries We Serve</h2>
      <div class="row">
        <div class="col-lg-4" v-for="industry in industries" :key="industry.id">
          <card
            :id="industry.id.toString()"
            :name="industry.name"
            :description="industry.description"
            :imageSource="industry.imageSource"
          />
        </div>
      </div>
    </section>   
    <span class="mb-2 pb-2"/>

    <nuxt-img 
          src="@/assets//austin-distel-7KsyY-Ed7B8-unsplash.jpg"
          alt="About Oplyst International" 
          class="about-image"
        />
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
</div>
</template>
<style lang="scss">
body {
  font-family: 'Goldman Sans', sans-serif;
  font-weight: 400; /* Using the regular weight for body text */
  font-size: 16px;
  line-height: 1.6;
  color: #333; /* Darker color for better contrast */
}
/*
.page-container {
  padding: 2rem;
}
*/
.hero {
  background-color: #f4f4f4;
}

.title, .subtitle {
  color: #003366;
}

#hero .title {
  font-size: 3rem; /* Made the title slightly bigger */
  font-weight: 700; /* Bold for emphasis */
}

#hero .subtitle {
  font-size: 1.8rem; /* Increased font size */
  margin-top: 1rem;
  font-weight: 400; /* Regular weight for better readability */
}

.tag {
  background-color: #003366;
  color: #ffffff;
  padding: 0.5rem;
  border-radius: 15px;
  transition: background-color 0.3s;
  
  &:hover {
    background-color: darken(#003366, 10%);
  }
}

.input, .button {
  border-radius: 5px;
  margin: 0.5rem 0;
}

.button {
  background-color: #003366;
  color: #ffffff;
  transition: background-color 0.3s;

  &:hover {
    background-color: darken(#003366, 10%);
  }
}

.row {
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
  
  .col-lg-4 {
    flex: 0 0 calc(33.3333% - 1rem);
    margin: 0.5rem;
  }
}

.about-image {
  max-width: 100%;
  margin-bottom: 20px;
  border-radius: 5px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.content {
  font-size: 1.4rem; /* Slightly larger font size */
  line-height: 1.7; /* Improved line spacing */
  color: #333333;
}

.subheader {
  box-sizing: content-box;
  padding: 1rem 0;
  border-top: 1px solid #e0e0e0;
  border-bottom: 1px solid #e0e0e0;
}
</style>
