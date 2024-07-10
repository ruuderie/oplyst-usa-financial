import type { Meta, StoryObj } from '@storybook/vue3';

import ToastProvider from '../components/ui/toast/ToastProvider.vue';

const meta = {
  title: 'ToastProvider',
  component: ToastProvider,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ToastProvider>;

export default meta;
type Story = StoryObj<typeof ToastProvider>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};