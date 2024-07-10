import type { Meta, StoryObj } from '@storybook/vue3';

//import Test from '../components/ui/accordion/Test.vue';
import Test from '../components/Test.vue';

const meta = {
  title: 'Test',
  component: Test,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Test>;

export default meta;
type Story = StoryObj<typeof Test>;

export const Primary: Story = {
  name: 'I am the primary',
  render: (args) => ({
    components: { Test },
    setup() {
      return { args };
    },
    template: '<Test  />',
  }),
  args: {}
};