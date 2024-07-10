import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogContent from '../components/ui/alert-dialog/AlertDialogContent.vue';

const meta = {
  title: 'AlertDialogContent',
  component: AlertDialogContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogContent>;

export default meta;
type Story = StoryObj<typeof AlertDialogContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};