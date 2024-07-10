import type { Meta, StoryObj } from '@storybook/vue3';

import DialogContent from '../components/ui/dialog/DialogContent.vue';

const meta = {
  title: 'DialogContent',
  component: DialogContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DialogContent>;

export default meta;
type Story = StoryObj<typeof DialogContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};