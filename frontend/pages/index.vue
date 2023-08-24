<script setup>
import { ref } from "vue";
import Card from "@/components/Card.vue";

// Define a reactive variable for the asset path
const logoSrc = ref("@/assets/Oplyst_International_White_logo_800x600.png");

// A method to check if the asset is available
// (this is a basic example, you might want a more sophisticated check in a real application)
const hasLogo = computed(() => {
  return logoSrc.value !== "";
});

const industries = [
  {
    id: 1,
    name: "Real Estate",
    description:
      "We help property experts get the best financial solutions, making your property projects successful.",
    imageSource: "",
    route: "industries/real-estate",
  },
  {
    id: 2,
    name: "Construction",
    description:
      "Building something great? Our innovative loans will help you every step of the way.",
    imageSource: "",
    route: "industries/construction",
  },
  {
    id: 3,
    name: "Healthcare",
    description:
      "We boost healthcare with smart financial solutions, helping you take care of what matters most.",
    imageSource: "",
    route: "industries/healthcare",
  },
  {
    id: 5,
    name: "Manufacturing",
    description:
      "Your production needs strong financial backing. We're here to power it up.",
    imageSource: "",
    route: "industries/manufacturing",
  },
  {
    id: 6,
    name: "Logistics",
    description:
      "For every owner operator out there, we provide financing that keeps you on the road and able to meet demand.",
    imageSource: "",
    route: "industries/retail",
  },
];

const services = [
  {
    id: 1,
    name: "Real Estate Loans",
    description:
      "Get the right funding for homes, offices, or construction projects. We're here to back your property dreams.",
    imageSource: "",
    route: "services/commercial-mortgages",
  },
  {
    id: 2,
    name: "Equipment Financing",
    description:
      "Don't let the cost of tools slow you down. We'll help you lease the equipment, so you can keep working.",
    imageSource: "",
    route: "services/equipment-loans",
  },
  {
    id: 3,
    name: "SBA Loans",
    description:
      "Small businesses deserve support. With our government-backed loans, you can fund any aspect of your venture.",
    imageSource: "",
    route: "services/sba-loans",
  },
  {
    id: 4,
    name: "Merchant Cash Advance",
    description: "Short on cash? We offer quick funds based on your future sales.",
    imageSource: "",
    route: "services/merchant-cash-advance",
  },
  {
    id: 6,
    name: "Healthcare Loans",
    description:
      "Whether it's for new equipment, expansion or payroll we have many specialized offerings by various lenders.",
    imageSource: "",
    route: "industries/healthcare",
  },
  {
    id: 5,
    name: "More Services",
    description:
      "View the entire range of loan products we can provide to help your business grow.",
    imageSource: "",
    route: "/services",
  },
];
</script>

<template>
  <NuxtLayout>
    <div class="">
      <div>
        <section class="hero is-fullwidth has-background-dark">
          <div class="overlay" />
          <div class="hero-body">
            <div class="container is-fluid">
              <NuxtLink
                target="_blank"
                class="tag"
                to="https://www.linkedin.com/company/oplystusa/"
              >
                @oplyst-international
              </NuxtLink>
              <h3 class="text-white text-2xl md:text-4xl lg:text-6xl font-bold">
                Elevate Your Ambitions with Direct, Fast, and Custom Business Financing.
              </h3>
            </div>
          </div>
        </section>


        <section class="section">
          <div class="w-full max-w-screen-lg block mx-auto">
            <div class="grid grid-cols-12 gap-3">
              <div class="col-span-12 md:col-span-4">
                <input
                  class="input is-medium !border-2 !border-gray-200 hover:!border-[#123262]"
                  type="text"
                  placeholder="Full Name"
                />
              </div>
              <div class="col-span-12 md:col-span-4">
                <input
                  class="input is-medium !border-2 !border-gray-200 hover:!border-[#123262]"
                  type="email"
                  placeholder="email@yourcompany.com"
                />
              </div>
              <div class="col-span-12 md:col-span-4">
                <button class="button is-info is-medium is-fullwidth">
                  Stay Informed
                </button>
              </div>
            </div>
          </div>
        </section>
        <!--  <h2 class="statement">
            Dynamic Lending and Financing for Modern Real Estate and Business Goals.
        </h2> -->
        <section class="section">
          <div class="container">
            <h2 class="statement has-text-centered">
              Tailored Business Funding Service with Precision Matching
            </h2>

            <div class="columns">
              <div class="column is-one-third">
                <h3 class="statement-subtitle">Swift and Simple Application Process</h3>
                <p>
                  Simply contact us at <a target="_blank" href="tel:860-333-6524" > (860)-333-6524 </a> or via email at <a target="_blank" href="mailto:loans@oplystusa.com" > loans@oplystusa.com </a>. 
                  Designed for businesses operational for at least 2 months and boasting a monthly
                  revenue of over $10,000, we ensure your time is well-spent.
                </p>
              </div>

              <div class="column is-one-third">
                <h3 class="statement-subtitle">
                  Expert Loan Underwriting for Your Profile
                </h3>
                <p>
                  Our seasoned underwriters meticulously review your application, ensuring
                  you're perfectly matched. Based on your unique profile, we connect you
                  to lenders who are actively seeking businesses with your exact
                  credentials.
                </p>
              </div>

              <div class="column is-one-third">
                <h3 class="statement-subtitle">Direct Connection to Suitable Lenders</h3>
                <p>
                  Gone are the days of one-size-fits-all loan offers. We specialize in
                  pairing you with lenders that match your profile, expediting approval
                  processes. With our precision-matching, you could potentially access
                  funds on the very same day of your application.
                </p>
              </div>
            </div>
          </div>
        </section>
        <section class="section">
          <div class="container">
            <h2 class="has-text-centered is-size-1 statement">How we help</h2>
            <div class="columns is-multiline">
              <div
                class="column is-one-third has-text-centered"
                v-for="service in services"
                :key="service.id"
              >
                <NuxtLink :to="service.route">
                  <Card
                    :id="service.id.toString()"
                    :name="service.name"
                    :description="service.description"
                    :imageSource="service.imageSource"
                    style="height: 100%"
                  />
                </NuxtLink>
              </div>
            </div>
          </div>
        </section>

        <section class="section">
          <div class="container">
            <h2 class="statement has-text-centered is-size-1">Industries We Serve</h2>
            <div class="columns is-multiline">
              <div
                class="column is-one-third"
                v-for="industry in industries"
                :key="industry.id"
              >
                <NuxtLink :to="industry.route">
                  <Card
                    :id="industry.id.toString()"
                    :name="industry.name"
                    :description="industry.description"
                    :imageSource="industry.imageSource"
                    style="height: 100%"
                  />
                </NuxtLink>
              </div>
            </div>
          </div>
        </section>
        <span class="mb-2 pb-2" />

        <nuxt-img
          src="@/assets//austin-distel-7KsyY-Ed7B8-unsplash.jpg"
          alt="About Oplyst International"
          class="about-image"
        />
        <section class="section is-medium is-mobile has-text-centered">
          <h1 class="statement">Our Values</h1>
          <ul>
            <li class="statement-subtitle">Innovation</li>
            <li class="statement-subtitle">Simplicity</li>
            <li class="statement-subtitle">Excellence</li>
            <li class="statement-subtitle">Privacy</li>
            <li class="statement-subtitle">Client Service</li>
            <li class="statement-subtitle">Integrity</li>
            <li class="statement-subtitle">Teamwork</li>
          </ul>
        </section>

        <section class="section is-medium">
          <div class="has-text-centered py-3">
            <a target="_blank" href="mailto:loans@oplystusa.com" >
              <button class="button is-large is-responsive">
                Schedule a Discovery Call
              </button>
            </a>
          </div>
          <div class="content is-large">
            <center><h3>Oplyst International</h3></center>
            <p>
              At Oplyst International, we are innovators, disruptors, and pioneers. Our
              unique combination of technology, logistics, and financial advisory allows
              us to provide holistic solutions to complex challenges. We do business with
              integrity, fostering long-term partnerships based on mutual trust and
              respect.
            </p>
            <p>
              With a firm commitment to simplicity, quality, and service, we prioritize
              client satisfaction and champion a future where businesses thrive. Oplyst
              International is more than just a company - we are shaping the future.
              Schedule a chat with us today to learn more about our vision, mission, and
              the services we offer.
            </p>
          </div>
        </section>
      </div>
    </div>
  </NuxtLayout>
</template>
<style lang="scss" scoped>
body {
  font-family: "Goldman Sans", sans-serif;
  font-weight: 400; // Using the regular weight for body text
  font-size: 16px;
  line-height: 1.6;
  color: #333; // Darker color for better contrast
}

.section {
  h3.subtitle {
    margin-bottom: 1rem;
    font-weight: bold;
  }

  p {
    font-size: 1rem;
  }
}
.hero {
  position: relative;
  background-color: #000000;
  // background-image: url("@/assets/salym.eth_Image_of_a_vintage_ship_with_billowing_sails_navigati_3e5a3024-b213-47d3-b9b1-6a5f4fe59c55.png");
  background-image: url("@/assets/declan_sun_j4ED0mgXp2c_unsplash.jpg");
  background-size: cover; // changed to cover so it fills the container
  background-repeat: no-repeat;
  background-position: center;
  //max-width: 200%;
  // max-height: 200%;
  // max-height: 640px;  // set max height to the height of your image
  overflow: hidden;
  height: 80vh;

  & .overlay {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    background: black;
    opacity: 0.4;
  }

  & .hero-body {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center; // This will make sure text inside the hero-body is also centered

    & .title {
      font-size: 3rem; // Made the title slightly bigger
      font-weight: 700; // Bold for emphasis
    }
  }

  @media (max-width: 768px) {
    // when the screen width is 768px or less
    max-height: 300px; // adjust based on your preference for mobile view
  }
}
.statement {
  color: #003366;
  margin-bottom: 15px;
  font-size: 1.5rem;
  font-weight: 800;
}
.statement-subtitle {
  color: #003366;
  margin-bottom: 15px;
  font-size: 1.2rem;
  font-weight: 800;
}
.hero-body {
  display: flex;
  flex-direction: column;
  align-items: top;
  justify-content: center;
  text-align: center; // This will make sure text inside the hero-body is also centered
}

.logo-img {
  margin-top: -350px;
  max-width: 30%;
  max-height: 30%;
}

.title,
.subtitle {
  color: #e6e9eb;
  outline: olive;
  outline-offset: 1cm;
  font-weight: 700;
  font-size: 1.5rem;

  @media (max-width: 768px) {
    font-size: 1.2rem;
  }
}

#hero .title {
  font-size: 3rem; // Made the title slightly bigger
  font-weight: 700; // Bold for emphasis

  @media (max-width: 768px) {
    font-size: 2rem;
  }
}

#hero .subtitle {
  font-size: 1.8rem; // Increased font size
  margin-top: 3rem;
  font-weight: 700; // Regular weight for better readability

  @media (max-width: 768px) {
    font-size: 1.4rem;
    margin-top: 2rem;
  }
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

.input,
.button {
  border-radius: 5px;
  margin: 0.5rem 0;

  @media (max-width: 768px) {
    width: 100%; // Making the input and button 100% width on mobile
    margin: 0.3rem 0;
  }
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

    @media (max-width: 768px) {
      flex: 0 0 100%; // This ensures the cards take the full width on mobile
      margin: 0.5rem 0; // Adjust margin as needed
    }
  }
}

.about-image {
  max-width: 100%;
  max-height: 20%;
  margin-bottom: 20px;
  border-radius: 5px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.content {
  font-size: 1.4rem; // Slightly larger font size
  line-height: 1.7; // Improved line spacing
  color: #333333;
}

.subheader {
  box-sizing: content-box;
  padding: 1rem 0;
  border-top: 1px solid #e0e0e0;
  border-bottom: 1px solid #e0e0e0;
}
</style>
