import type { Meta, StoryObj } from '@storybook/vue3';

import DialogClose from '../components/ui/dialog/DialogClose.vue';

const meta = {
  title: 'DialogClose',
  component: DialogClose,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DialogClose>;

export default meta;
type Story = StoryObj<typeof DialogClose>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};