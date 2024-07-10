import type { Meta, StoryObj } from '@storybook/vue3';

import ToastClose from '../components/ui/toast/ToastClose.vue';

const meta = {
  title: 'ToastClose',
  component: ToastClose,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ToastClose>;

export default meta;
type Story = StoryObj<typeof ToastClose>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};