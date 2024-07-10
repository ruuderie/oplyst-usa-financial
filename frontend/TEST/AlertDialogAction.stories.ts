import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogAction from '../components/ui/alert-dialog/AlertDialogAction.vue';

const meta = {
  title: 'AlertDialogAction',
  component: AlertDialogAction,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogAction>;

export default meta;
type Story = StoryObj<typeof AlertDialogAction>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};