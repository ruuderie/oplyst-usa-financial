import type { Meta, StoryObj } from '@storybook/vue3';

import TheNavBar from '../components/TheNavBar.vue';

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
const meta = {
  title: 'TheNavBar',
  component: TheNavBar,
  // This component will have an automatically generated docsPage entry: https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TheNavBar>;

export default meta;
type Story = StoryObj<typeof TheNavBar>;
/*
 *👇 Render functions are a framework specific feature to allow you control on how the component renders.
 * See https://storybook.js.org/docs/api/csf
 * to learn how to use render functions.
 */
 export const Primary: Story = {
    // Use the `render` key to define the story's rendering
    /*render: () => ({
      // Define the template for your story
      template: '<TheNavBar class="page-container__navbar navbar container is-fluid h-28" />'
    }), */
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
  };
  