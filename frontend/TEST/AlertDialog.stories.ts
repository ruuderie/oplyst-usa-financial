import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialog from '../components/ui/alert-dialog/AlertDialog.vue';

const meta = {
  title: 'AlertDialog',
  component: AlertDialog,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialog>;

export default meta;
type Story = StoryObj<typeof AlertDialog>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};