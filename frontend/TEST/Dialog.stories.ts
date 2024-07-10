import type { Meta, StoryObj } from '@storybook/vue3';

import Dialog from '../components/ui/dialog/Dialog.vue';

const meta = {
  title: 'Dialog',
  component: Dialog,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Dialog>;

export default meta;
type Story = StoryObj<typeof Dialog>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};