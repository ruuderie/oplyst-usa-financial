import type { Meta, StoryObj } from '@storybook/vue3';

import Toast from '../components/ui/toast/Toast.vue';

const meta = {
  title: 'Toast',
  component: Toast,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Toast>;

export default meta;
type Story = StoryObj<typeof Toast>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};