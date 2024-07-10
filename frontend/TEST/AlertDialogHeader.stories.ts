import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogHeader from '../components/ui/alert-dialog/AlertDialogHeader.vue';

const meta = {
  title: 'AlertDialogHeader',
  component: AlertDialogHeader,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogHeader>;

export default meta;
type Story = StoryObj<typeof AlertDialogHeader>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};