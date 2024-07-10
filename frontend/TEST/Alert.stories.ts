import type { Meta, StoryObj } from '@storybook/vue3';

import Alert from '../components/ui/alert/Alert.vue';

const meta = {
  title: 'Alert',
  component: Alert,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Alert>;

export default meta;
type Story = StoryObj<typeof Alert>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};