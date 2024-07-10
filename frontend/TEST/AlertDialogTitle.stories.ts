import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogTitle from '../components/ui/alert-dialog/AlertDialogTitle.vue';

const meta = {
  title: 'AlertDialogTitle',
  component: AlertDialogTitle,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogTitle>;

export default meta;
type Story = StoryObj<typeof AlertDialogTitle>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};