import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogDescription from '../components/ui/alert-dialog/AlertDialogDescription.vue';

const meta = {
  title: 'AlertDialogDescription',
  component: AlertDialogDescription,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogDescription>;

export default meta;
type Story = StoryObj<typeof AlertDialogDescription>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};