import type { Meta, StoryObj } from '@storybook/vue3';

import ToastViewport from '../components/ui/toast/ToastViewport.vue';

const meta = {
  title: 'ToastViewport',
  component: ToastViewport,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ToastViewport>;

export default meta;
type Story = StoryObj<typeof ToastViewport>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};