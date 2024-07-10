import type { Meta, StoryObj } from '@storybook/vue3';

import SelectTrigger from '../components/ui/select/SelectTrigger.vue';

const meta = {
  title: 'SelectTrigger',
  component: SelectTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectTrigger>;

export default meta;
type Story = StoryObj<typeof SelectTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};