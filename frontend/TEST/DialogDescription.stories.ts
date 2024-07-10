import type { Meta, StoryObj } from '@storybook/vue3';

import DialogDescription from '../components/ui/dialog/DialogDescription.vue';

const meta = {
  title: 'DialogDescription',
  component: DialogDescription,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DialogDescription>;

export default meta;
type Story = StoryObj<typeof DialogDescription>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};