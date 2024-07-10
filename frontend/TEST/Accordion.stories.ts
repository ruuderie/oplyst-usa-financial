import type { Meta, StoryObj } from '@storybook/vue3';

//import Accordion from '../components/ui/accordion/Accordion.vue';
import Accordion from '../components/ui/accordion/Accordion.vue';

const meta = {
  title: 'Accordion',
  component: Accordion,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Accordion>;

export default meta;
type Story = StoryObj<typeof Accordion>;

export const Primary: Story = {
    args: {
       // class: 'page-container__navbar navbar container is-fluid h-28',
    }
};