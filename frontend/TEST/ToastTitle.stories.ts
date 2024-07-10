import type { Meta, StoryObj } from '@storybook/vue3';

import ToastTitle from '../components/ui/toast/ToastTitle.vue';

const meta = {
  title: 'ToastTitle',
  component: ToastTitle,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ToastTitle>;

export default meta;
type Story = StoryObj<typeof ToastTitle>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};