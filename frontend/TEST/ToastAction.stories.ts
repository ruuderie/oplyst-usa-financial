import type { Meta, StoryObj } from '@storybook/vue3';

import ToastAction from '../components/ui/toast/ToastAction.vue';

const meta = {
  title: 'ToastAction',
  component: ToastAction,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ToastAction>;

export default meta;
type Story = StoryObj<typeof ToastAction>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};