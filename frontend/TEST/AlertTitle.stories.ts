import type { Meta, StoryObj } from '@storybook/vue3';

import AlertTitle from '../components/ui/alert/AlertTitle.vue';

const meta = {
  title: 'AlertTitle',
  component: AlertTitle,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertTitle>;

export default meta;
type Story = StoryObj<typeof AlertTitle>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};