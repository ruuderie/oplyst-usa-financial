import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogCancel from '../components/ui/alert-dialog/AlertDialogCancel.vue';

const meta = {
  title: 'AlertDialogCancel',
  component: AlertDialogCancel,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogCancel>;

export default meta;
type Story = StoryObj<typeof AlertDialogCancel>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};